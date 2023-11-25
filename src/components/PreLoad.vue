<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";

const router = useRouter();

const fetchData = async () => {
    const sessionAlive = await invoke("session_alive", { session: "fewrfgreg" });
    return sessionAlive;
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
    <div class="card flex align-items-center justify-content-center" style="height: 100%; widows: 100%;">
        <ProgressSpinner style="width: 17%; height: 17%" strokeWidth="4" animationDuration="1s" aria-label="预加载..." />
    </div>
</template>

<style scoped></style>