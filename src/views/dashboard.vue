<script setup lang="ts">
import PanelMenu from 'primevue/panelmenu';
import Button from 'primevue/button';
import Avatar from 'primevue/avatar';
import Divider from 'primevue/divider';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Toast from 'primevue/toast';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import Menu from 'primevue/menu';

import Settings from '../components/Settings.vue';
import JoinCommunityPanel from '../components/JoinCommunityPanel.vue'
import NewCommunityPanel from '../components/NewCommunityPanel.vue'
import Main from '../components/Main.vue'

import { onMounted, ref } from 'vue';
import { useToast } from 'primevue/usetoast';
import { useSessionsStore } from '../stores/sessions';
import { useRouter } from 'vue-router';

const router = useRouter()
const screenWidth = ref<number>(window.innerWidth)
const mobile = ref<boolean>(false)
function onResize() {
    screenWidth.value = window.innerWidth
    if (screenWidth.value < 600) {
        mobile.value = true
    } else {
        mobile.value = false
    }
}
function onClose() {
    selectedSession.value = null
    router.go(-1)
    router.push({ path: '/dashboard', replace: true })
}
window.onresize = onResize;
window.onpopstate = onClose;

const sessionsStore = useSessionsStore()

const changeSession = (item: any) => {
    return () => {
        selectedSession.value = item
        router.push("/dashboard?page=")
    }
}

function sessionChanged() {
    sessions.value[1].items = sessionsStore.communities.map((item: any) => {
        return {
            ...item,
            label: item.name,
            icon: 'pi pi-users',
            command: changeSession(item)
        };
    });
}

const sessions = ref([
    {
        label: '信道',
        icon: 'pi pi-inbox',
        items: [

        ]
    },
    {
        label: '社群',
        icon: 'pi pi-users',
        items: [

        ]
    }
])

const showDialog = ref<boolean>(false)
const showCreateCommunityPanel = ref<boolean>(false)
const showJoinCommunityPanel = ref<boolean>(false)
const toast = useToast()

const selectedSession = ref()
const menu = ref();

const toggle = (event: any) => {
    menu.value.toggle(event);
}

const items = ref([
    {
        label: '信道',
        items: [
            {
                label: '连接信道',
                icon: 'pi pi-link',
                command: () => {
                    toast.add({ 'severity': 'warn', 'summary': '中止', 'detail': '弦月测试期间不开放信道测试接口。', 'life': 3000 })
                }
            },
        ]
    },
    {
        label: '通讯',
        items: [
            {
                label: '创建社群',
                icon: 'pi pi-comments',
                command: () => showCreateCommunityPanel.value = true
            },
            {
                label: '添加社群',
                icon: 'pi pi-user-plus',
                command: () => showJoinCommunityPanel.value = true
            }
        ]
    }
]);

onMounted(() => {
    console.table(sessionsStore.communities)
    onResize()
    sessionChanged()
})
</script>

<template>
    <div class="w-full h-full">
        <Toast style="max-width: 90%;"></Toast>
        <Dialog v-model:visible="showCreateCommunityPanel" modal class="max-w-full" style="width: 30rem;">
            <template #header>
                <div class="inline-flex align-items-center justify-content-center gap-2">
                    <span class="font-bold white-space-nowrap">创建社群</span>
                </div>
            </template>
            <NewCommunityPanel @on-close="showCreateCommunityPanel = false; sessionChanged()"></NewCommunityPanel>
        </Dialog>
        <Dialog v-model:visible="showJoinCommunityPanel" modal class="max-w-full" style="width: 30rem;">
            <template #header>
                <div class="inline-flex align-items-center justify-content-center gap-2">
                    <span class="font-bold white-space-nowrap">加入社群</span>
                </div>
            </template>
            <JoinCommunityPanel></JoinCommunityPanel>
        </Dialog>
        <div class="flex flex-row h-full">
            <div :class="[
            'flex flex-column justify-content-between align-items-center bg-surface-300 border-right-2 border-200',
            ((mobile && selectedSession) ? 'hidden' : ''),
            (mobile ? 'p-2' : 'p-3')]">
                <div class="flex flex-column justify-content-center align-items-center">
                    <Avatar :label="'苏'"></Avatar>
                    <Divider layout="horizontal"></Divider>
                    <Button @click="toast.add({
            'summary': '暂未开放', 'detail': '弦月测试期间插件系统暂不开放，暂不支持增删插件！', 'life': 3000, 'severity': 'warn'
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
                        <Settings></Settings>
                    </Dialog>
                </div>
            </div>
            <div class="w-full">
                <Splitter class="w-full h-full border-none">
                    <SplitterPanel :size="26" style="min-width: 12rem;"
                        :class="[((mobile && selectedSession) ? 'hidden' : '')]">
                        <div class="w-full h-full flex flex-column gap-3 p-2">
                            <div class="inline-flex justify-content-between gap-2 max-w-full">
                                <IconField iconPosition="left">
                                    <InputIcon class="pi pi-search"></InputIcon>
                                    <InputText id="search" placeholder="搜索" class="w-full"></InputText>
                                </IconField>
                                <Button type="button" @click="toggle" icon="pi pi-plus" size="small"
                                    aria-haspopup="true" aria-controls="overlay_menu" outlined></Button>
                                <Menu ref="menu" id="overlay_menu" :model="items" :popup="true"></Menu>
                            </div>
                            <Divider class="m-0"></Divider>
                            <PanelMenu :model="sessions" multiple class="w-full">
                                <template #item="{ item, root }">
                                    <a v-ripple class="p-ripple flex align-items-center px-3 cursor-pointer border-round"
                                        :class="[(root ? 'py-2' : 'py-2'), ((!root && selectedSession && selectedSession.id == item.id) ? 'bg-primary-reverse' : '')]">
                                        <span :class="[item.icon, 'text-primary']"></span>
                                        <span :class="['ml-2', { 'font-semibold': item.items }]">{{ item.label }}</span>
                                    </a>
                                </template>
                            </PanelMenu>
                        </div>
                    </SplitterPanel>
                    <SplitterPanel :size="74" :class="[((mobile && !selectedSession) ? 'hidden' : '')]" :minSize="50">
                        <div v-if="!selectedSession"
                            class="w-full h-full flex flex-column justify-content-center align-items-center">
                            <img src="/icon.png" width="300"></img>
                        </div>
                        <Main v-else :session="selectedSession" @on-close="onClose" :mobile="mobile"
                            class="w-full h-full">
                        </Main>
                    </SplitterPanel>
                </Splitter>
            </div>
        </div>
    </div>
</template>

<style scoped>
:deep(.p-divider .p-divider-horizontal:before) {
    border-top-width: 2px;
}

:deep(.p-splitter-gutter-handle[aria-orientation="horizontal"]) {
    width: 2px !important;
}

:deep(.p-splitter-horizontal > .p-splitter-gutter) {
    cursor: ew-resize;
}

:deep(.hidden) {
    display: none !important;
}

:deep(.p-panelmenu .p-panelmenu-content) {
    padding: 0 0.25rem 0 0.25rem;
}

@media (max-width: 600px) {
    :deep(.p-splitter-gutter) {
        display: none;
    }
}
</style>
