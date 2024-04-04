<script setup lang="ts">
import { onMounted } from 'vue';
import { isLightTheme, useThemeStore } from './stores/theme';
import { usePrimeVue } from 'primevue/config';

const themeStore = useThemeStore();
const PrimeVue = usePrimeVue();

onMounted(() => {
    if ((themeStore.dark && isLightTheme()) || (!themeStore.dark && !isLightTheme()))
        themeStore.changeTheme(PrimeVue);
})
</script>

<template>
    <RouterView />
</template>

<style scoped></style>