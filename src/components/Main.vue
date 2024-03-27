<script setup lang="ts">
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Sidebar from 'primevue/sidebar';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Textarea from 'primevue/textarea';
import Listbox from 'primevue/listbox';
import Badge from 'primevue/badge';
import Toast from 'primevue/toast';
import { ref } from 'vue';
import { useToast } from 'primevue/usetoast';

const emit = defineEmits(['onClose'])
const toast = useToast()

defineModel('session');
const mobile = defineModel('mobile');

if (mobile.value === null) {
    mobile.value = true
}

const showSidebar = ref<boolean>(false)
const input = ref<string | null>()

const selectedMessage = ref();
const messages = ref([])

async function render(text: string) {
    const marked = await import('marked')
    if (text.split("\n\n").length <= 1) return text
    else return marked.marked(text)
}

async function sendMessage() {
    toast.add({ 'severity': 'warn', 'summary': '暂未开放', 'detail': '消息接口暂未开放！', 'life': 3000 })
}
</script>

<template>
    <div class="w-full h-full flex flex-column">
        <Toast style="max-width: 90%;"></Toast>
        <Toolbar class="border-noround border-left-none border-top-none p-2">
            <template #start>
                <div :class="['inline-flex gap-3 align-items-center', (mobile ? '' : 'pl-3')]">
                    <Button v-if="mobile" @click="emit('onClose')" icon="pi pi-arrow-left" plain text></Button>
                    <span>{{ session.label }}</span>
                </div>
            </template>
            <template #end>
                <Button @click="showSidebar = true" icon="pi pi-ellipsis-h" plain text></Button>
            </template>
        </Toolbar>
        <Sidebar v-model:visible="showSidebar" :header="session.label" position="right" class="max-w-full">
            <p>此处功能区为弦月测试范围，但是尚未实装。</p>
        </Sidebar>
        <div class="flex flex-column w-full h-full">
            <div class="card w-full flex justify-content-center h-full">
                <Listbox v-model="selectedMessage" :options="messages" optionLabel="text" optionValue="sequence"
                    optionGroupLabel="user.nickname" optionGroupChildren="items" class="w-full h-full border-none">
                    <template #empty>
                        <div class="flex align-items-center justify-content-center">
                            <div class="text-gray-500">开启一场私密对话吧！</div>
                        </div>
                    </template>
                    <template #optiongroup="slot">
                        <div class="flex align-items-center">
                            <div>{{ slot.option.user.nickname }}</div>
                        </div>
                    </template>
                    <template #option="slot">
                        <div v-tooltip.bottom="slot.option.timestamp">
                            <div class="flex text-white">
                                <div v-html="render(slot.option.text)"></div>
                                <div class="flex align-items-center align-self-end pb-1" style="margin-left: auto;">
                                    <Badge v-if="slot.option.status === 'sending'"
                                        class="bg-primary flex justify-content-center align-items-center pi pi-spin pi-spinner">
                                    </Badge>
                                    <Badge v-else-if="slot.option.status === 'failed'"
                                        class="bg-red-900 flex justify-content-center align-items-center pi pi-times">
                                    </Badge>
                                    <Badge v-else-if="slot.option.status === 'verified'"
                                        class="bg-green-900 flex justify-content-center align-items-center pi pi-verified">
                                    </Badge>
                                    <Badge v-else-if="slot.option.status === 'check'"
                                        class="bg-green-900 flex justify-content-center align-items-center pi pi-check">
                                    </Badge>
                                </div>
                            </div>
                        </div>
                    </template>
                </Listbox>
            </div>
            <div class="w-full border-top-2 border-200">
                <div
                    :class="['w-full h-full inline-flex justify-content-center align-items-center', (mobile ? 'p-2 gap-2' : 'p-3 gap-3')]">
                    <IconField class="w-full h-full" icon-position="left">
                        <InputIcon class="pi pi-lock"></InputIcon>
                        <Textarea rows="1" maxlength="512" v-model="input" auto-resize class="w-full"
                            placeholder="发送加密信息"></Textarea>
                    </IconField>
                    <div class="flex justify-content-end flex-column h-full">
                        <Button @click="sendMessage" icon="pi pi-send"></Button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
:deep(textarea) {
    resize: none;
    line-height: 1.5;
    vertical-align: middle;
    max-height: 12em !important;
}
</style>