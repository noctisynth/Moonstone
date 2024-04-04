<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import ProgressSpinner from "primevue/progressspinner";
import { useDebugStore } from "../stores/debug";
import { useLoginStore } from "../stores/login";

const router = useRouter();
const debugstore = useDebugStore()
const loginstore = useLoginStore()

const info = ref<string | null>();
const error = ref<string>();

const fetchData = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }

    if (debugstore.debug) {
        return true
    }

    let callback: { status: boolean; is_alive: boolean; error: string };

    if (loginstore.session_key != null && loginstore.node != null) {
        info.value = "登录中...";
        callback = JSON.parse(
            await invoke("session_alive", { server: loginstore.node, sessionkey: loginstore.session_key })
        );
        if (callback.status) {
            if (!callback.is_alive) {
                info.value = null;
                error.value = "登录验证失败，请重新登陆！";
                sendNotification({ title: '月长石', body: error.value });
                loginstore.session_key = null
                loginstore.isLoggedIn = false
                await new Promise(resolve => setTimeout(resolve, 2000));
                return false;
            } else {
                info.value = "登录验证成功！";
                loginstore.isLoggedIn = true
                return true;
            }
        } else {
            info.value = null;
            error.value = callback.error;
            sendNotification({ title: '月光石', body: error.value });
            await new Promise(resolve => setTimeout(resolve, 2000));
            return false;
        }
    } else {
        return false;
    };
};

fetchData().then((sessionAlive) => {
    if (!sessionAlive) {
        router.push({ path: "/login" });
    } else {
        router.push({ path: "/dashboard" });
    }
});
</script>

<template>
    <div class="w-full h-full">
        <div class="card flex align-items-center justify-content-center" style="padding-top: 39vh;">
            <ProgressSpinner style="width: 17%; height: 17%" strokeWidth="3" animationDuration="1s"
                aria-label="预加载..." />
        </div>
        <div class="card flex align-items-center justify-content-center" style="padding-top: 20vh;">
            <small class="p-info" id="text-info">{{ info || null }}</small>
            <small class="p-error" id="text-error">{{ error || null }}</small>
        </div>
    </div>
</template>

<style scoped></style>