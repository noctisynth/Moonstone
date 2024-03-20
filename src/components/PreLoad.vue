<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import ProgressSpinner from "primevue/progressspinner";

const router = useRouter();
const info = ref<string>();
const error = ref<string>();

const fetchData = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        await requestPermission();
    }
    sendNotification({ title: '月长石', body: "登录中..." });

    info.value = "登录中...";
    let callback: { status: boolean; is_alive: boolean; error: string };

    if (localStorage.getItem("session_key") != null && localStorage.getItem("server") != null) {


        callback = JSON.parse(
            await invoke("session_alive", { server: localStorage.getItem("server"), sessionkey: localStorage.getItem("session_key") })
        );
        if (callback.status) {
            if (!callback.is_alive) {
                info.value = "";
                error.value = "登录验证失败，请重新登陆！";
                sendNotification({ title: '月长石', body: error.value });
                localStorage.removeItem("session_key");
                localStorage.removeItem("isLoggedIn");
                await new Promise(resolve => setTimeout(resolve, 2000));
                return false;
            } else {
                info.value = "登录验证成功！";
                sendNotification({ title: '月光石', body: info.value });
                return true;
            }
        } else {
            info.value = "";
            error.value = callback["error"];
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
        localStorage.removeItem("isLoggedIn");
        router.push({ path: "/login" });
    } else {
        localStorage.setItem("isLoggedIn", "true");
        router.push({ path: "/dashboard" });
    }
});
</script>

<template>
    <div class="card flex align-items-center justify-content-center" style="padding-top: 39vh;">
        <ProgressSpinner style="width: 17%; height: 17%" strokeWidth="4" animationDuration="1s" aria-label="预加载..." />
    </div>
    <div class="card flex align-items-center justify-content-center" style="padding-top: 20vh;">
        <small class="p-info" id="text-info">{{ info || null }}</small>
        <small class="p-error" id="text-error">{{ error || null }}</small>
    </div>
</template>

<style scoped></style>