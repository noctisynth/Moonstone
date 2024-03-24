<script setup lang="ts">
import InputSwitch from 'primevue/inputswitch';
import Button from 'primevue/button';

import { useThemeStore } from '../stores/theme'
import { useLoginStore } from '../stores/login'
import { usePrimeVue } from 'primevue/config';
import { useRouter } from 'vue-router';

const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const loginstore = useLoginStore()
const router = useRouter()
</script>

<template>
    <div class="flex flex-column justify-content-center align-items-center">
        <div class="flex flex-column w-full p-2">
            <div class="inline-flex justify-content-between">
                <div><span>风格</span></div>
                <InputSwitch v-model="themestore.dark" @click="themestore.changeTheme(PrimeVue)">
                </InputSwitch>
            </div>
            <div class="mt-6 flex justify-content-end">
                <Button @click="loginstore.logout(); router.push('/')" icon="pi pi-sign-out" label="退出登录"
                    size="small"></Button>
            </div>
        </div>
    </div>
</template>

<style scoped></style>