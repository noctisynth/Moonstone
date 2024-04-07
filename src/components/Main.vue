<script setup lang="ts">
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Sidebar from 'primevue/sidebar';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Textarea from 'primevue/textarea';
import Listbox from 'primevue/listbox';
import Avatar from 'primevue/avatar';
import { onMounted, ref } from 'vue';
import { useToast } from 'primevue/usetoast';
import { generateRandomString } from '../scripts/random';

const emit = defineEmits(['onClose'])
const toast = useToast()
let marked: any

defineModel('session');
const mobile = defineModel('mobile');

if (mobile.value === null) {
    mobile.value = true
}

const showSidebar = ref<boolean>(false)
const input = ref<string | null>()

const selectedMessage = ref();
const messages = ref<{
    user: {
        id: string,
        nickname: string,
    },
    items: {
        id: string,
        text: string,
        status: string
    }[]
}[]>([])

function render(text: string) {
    if (text.split("\n\n").length <= 1) return text
    else return marked.marked(text)
}

function updateStatus(id: string, status: string) {
    let messageIndex: number = -1
    let itemIndex: number = -1;
    messages.value.forEach(message => {
        message.items.forEach(item => {
            if (item.id === id) {
                messageIndex = messages.value.indexOf(message)
                itemIndex = message.items.indexOf(item)
            }
        });
    });
    if (messageIndex === -1 || itemIndex === -1)
        return toast.add({ 'severity': 'error', 'summary': '错误', 'detail': '消息不存在！', 'life': 3000 })
    messages.value[messageIndex].items[itemIndex].status = status
}

async function sendMessage(value: any) {
    toast.add({ 'severity': 'warn', 'summary': '暂未开放', 'detail': '消息接口暂未开放！', 'life': 3000 })
    console.log(value.id)
    updateStatus(value.id, "failed")
}

async function onSend() {
    if (!input.value) return
    if (messages.value.length == 0)
        messages.value.push({
            user: {
                id: 'ssss',
                nickname: '苏向夜',
            },
            items: []
        })

    const message = {
        id: generateRandomString(10),
        text: input.value,
        status: 'sending',
    }

    if (messages.value[messages.value.length - 1].user.id === "ssss")
        messages.value[messages.value.length - 1].items.push(message)
    else
        messages.value.push({
            user: {
                id: 'ssss',
                nickname: '苏向夜',
            },
            items: [message]
        })
    sendMessage(message);
    console.table(messages.value)
    input.value = null;
}

onMounted(async () => {
    marked = await import('marked')
})
</script>

<template>
    <div class="w-full h-full flex flex-column">
        <Toolbar class="border-noround border-left-none border-top-none p-2">
            <template #start>
                <div :class="['inline-flex align-items-center', (mobile ? 'gap-1' : 'pl-3 gap-3')]">
                    <Button v-if="mobile" @click="emit('onClose')" icon="pi pi-arrow-left" plain text></Button>
                    <span>{{ session.name }}</span>
                </div>
            </template>
            <template #end>
                <Button @click="showSidebar = true" icon="pi pi-ellipsis-h" plain text></Button>
            </template>
        </Toolbar>
        <Sidebar v-model:visible="showSidebar" :header="session.name" position="right" class="max-w-full">
            <p>此处功能区为弦月测试范围，但是尚未实装。</p>
        </Sidebar>
        <div class="flex flex-column w-full h-full">
            <div class="card w-full flex justify-content-center h-full">
                <Listbox v-model="selectedMessage" :options="messages" optionLabel="text" optionValue="id"
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
                            <div class="flex text-white flex-row justify-content-between">
                                <div v-html="render(slot.option.text)"></div>
                                <div class="flex align-items-center align-self-end py-1">
                                    <Avatar v-if="slot.option.status === 'sending'" icon="pi pi-spin pi-spinner text-xs"
                                        class="bg-primary w-1rem h-1rem" shape="circle"></Avatar>
                                    <Avatar v-else-if="slot.option.status === 'failed'" icon="pi pi-times text-xs"
                                        class="bg-red-900 w-1rem h-1rem" shape="circle">
                                    </Avatar>
                                    <Avatar v-else-if="slot.option.status === 'verified'" icon="pi pi-verified text-xs"
                                        class="bg-green-900 w-1rem h-1rem" shape="circle">
                                    </Avatar>
                                    <Avatar v-else-if="slot.option.status === 'check'" icon="pi pi-check"
                                        class="bg-green-900 w-1rem h-1rem" shape="circle">
                                    </Avatar>
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
                        <Textarea name="message" rows="1" maxlength="512" v-model="input" auto-resize class="w-full"
                            placeholder="发送加密信息"></Textarea>
                    </IconField>
                    <div class="flex justify-content-end flex-column h-full">
                        <Button @click="onSend" icon="pi pi-send"></Button>
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