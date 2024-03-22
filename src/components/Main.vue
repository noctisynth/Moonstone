<script setup lang="ts">
import PanelMenu from 'primevue/panelmenu';
import Button from 'primevue/button';
import Avatar from 'primevue/avatar';
import Divider from 'primevue/divider';
import Dialog from 'primevue/dialog';
import InputSwitch from 'primevue/inputswitch';
import InputText from 'primevue/inputtext';
import Toast from 'primevue/toast';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import { ref } from 'vue';
import { useThemeStore } from '../stores/theme';
import { usePrimeVue } from 'primevue/config';
import { useToast } from 'primevue/usetoast';
import { useLoginStore } from '../stores/login';
import { useRouter } from 'vue-router';

const router = useRouter()
const loginstore = useLoginStore()

const items = ref([
    {
        label: '信道',
        icon: 'pi pi-inbox',
        items: [

        ]
    },
    {
        label: '群组',
        icon: 'pi pi-users',
        items: [

        ]
    }
])

const showDialog = ref<boolean>(false)
const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const toast = useToast()
</script>

<template>
    <div class="w-full h-full">
        <Toast></Toast>
        <div class="flex flex-row h-full">
            <div
                class="flex flex-column justify-content-between align-items-center p-3 bg-surface-300 border-right-2 border-200">
                <div class="flex flex-column justify-content-center align-items-center">
                    <Avatar :label="'苏'"></Avatar>
                    <Divider layout="horizontal"></Divider>
                    <Button @click="toast.add({
                        'summary': '暂未开放', 'detail': '插件系统暂未开放，暂不支持增删插件！', 'life': 3000, 'severity': 'warn'
                    })" icon="pi pi-plus" plain text></Button>
                </div>
                <div class="flex flex-column gap-3">
                    <Button @click="showDialog = true" icon="pi pi-cog" outlined></Button>
                    <Dialog v-model:visible="showDialog" modal class="max-w-full" style="width: 30rem;">
                        <template #header>
                            <div class="inline-flex align-items-center justify-content-center gap-2">
                                <span class="font-bold white-space-nowrap">设置</span>
                            </div>
                        </template>
                        <div class="flex flex-column justify-content-center align-items-center">
                            <div class="flex flex-column w-full p-2">
                                <div class="inline-flex justify-content-between">
                                    <div><span>风格</span></div>
                                    <InputSwitch v-model="themestore.dark" @click="themestore.changeTheme(PrimeVue)">
                                    </InputSwitch>
                                </div>
                                <div class="mt-6 flex justify-content-end">
                                    <Button @click="loginstore.logout(); router.push('/')" icon="pi pi-sign-out" label="退出登录" size="small"></Button>
                                </div>
                            </div>
                        </div>
                    </Dialog>
                </div>
            </div>
            <div class="w-full p-2">
                <div class="w-full h-full flex flex-column gap-3 p-1">
                    <div class="inline-flex justify-content-between gap-3 max-w-full">
                        <IconField iconPosition="left">
                            <InputIcon class="pi pi-search"></InputIcon>
                            <InputText placeholder="搜索" class="w-full"></InputText>
                        </IconField>
                        <Button icon="pi pi-plus" size="small" outlined></Button>
                    </div>
                    <Divider class="m-0"></Divider>
                    <PanelMenu :model="items" multiple class="w-full"></PanelMenu>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
:deep(.p-divider.p-divider-horizontal:before) {
    border-top-width: 2px;
}
</style>
