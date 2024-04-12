<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import ProgressSpinner from "primevue/progressspinner";
import { useDebugStore } from "../stores/debug";
import { useLoginStore } from "../stores/login";
import { useSessionsStore } from "../stores/sessions";

const router = useRouter();
const debugstore = useDebugStore();
const loginStore = useLoginStore();
const sessionsStore = useSessionsStore();

const info = ref<string | null>();
const error = ref<string>();

const fetchData = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }

    if (debugstore.debug) {
        return true;
    }

    if (loginStore.session_key != null && loginStore.node != null) {
        info.value = "登录中...";
        const callback: { status: boolean; is_alive: boolean; error: string } = await invoke("session_alive", { server: loginStore.node, sessionkey: loginStore.session_key });
        if (callback.status) {
            if (!callback.is_alive) {
                info.value = null;
                error.value = "登录验证失败，请重新登陆！";
                sendNotification({ title: '月长石', body: error.value });
                loginStore.session_key = null;
                loginStore.isLoggedIn = false;
                await new Promise(resolve => setTimeout(resolve, 2000));
                return false;
            } else {
                info.value = "登录验证成功，同步数据中...";
                const res: { status: boolean, error: string, profile: any } = await invoke("get_account_profile", {
                    node: loginStore.node,
                    token: loginStore.session_key
                })
                if (res.status) {
                    loginStore.user_id = res.profile.id;
                    loginStore.userProfile = res.profile.profile;
                    loginStore.isLoggedIn = true;
                    return true;
                } else {
                    info.value = null
                    error.value = res.error
                    await new Promise(resolve => setTimeout(resolve, 2000));
                    return false
                }
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
        sessionsStore.init().then(() => {
            router.push({ path: "/dashboard" });
        });
    }
});
</script>

<template>
    <div class="w-full h-full ">
        <div class="flex flex-column h-full p-8 align-items-center justify-content-center">
            <ProgressSpinner class="w-full h-full w-3" animationDuration="1s" aria-label="预加载..." />
            <div class="flex align-items-center justify-content-center">
                <small :class="[(info ? 'p-info' : 'p-error')]" id="text-info">{{ info || error }}</small>
            </div>
        </div>
    </div>
</template>

<style scoped></style>