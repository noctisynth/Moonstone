<script setup lang="ts">
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Sidebar from 'primevue/sidebar';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Textarea from 'primevue/textarea';
import Listbox from 'primevue/listbox';
import Badge from 'primevue/badge';
import MarkdownIt from 'markdown-it';
import { ref } from 'vue';

const emit = defineEmits(['onClose'])
const md = new MarkdownIt();

defineModel('session');
const mobile = defineModel('mobile');

if (mobile.value === null) {
    mobile.value = true
}

const showSidebar = ref<boolean>(false)
const input = ref<string | null>()

const selectedMessage = ref();
const messages = ref([])

function render(text: string) {
    if (text.split("\n\n").length <= 1) return text
    else return md.render(text)
}
</script>

<template>
    <div class="w-full h-full flex flex-column">
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
        <div class="w-full h-full">
            <Splitter class="w-full h-full" layout="vertical">
                <SplitterPanel :size="80" :min-size="40">
                    <div class="card w-full flex justify-content-center h-full">
                        <Listbox v-model="selectedMessage" :options="messages" optionLabel="text" optionValue="sequence"
                            optionGroupLabel="user.nickname" optionGroupChildren="items"
                            class="w-full h-full border-none">
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
                                        <div class="flex align-items-center align-self-end pb-1"
                                            style="margin-left: auto;">
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
                </SplitterPanel>
                <SplitterPanel :size="10" style="min-height: 4.6rem;">
                    <div class="w-full h-full inline-flex gap-3 justify-content-center align-items-center p-3"
                        style="min-height: fit-content;">
                        <IconField class="w-full h-full max-h-full" icon-position="left"
                            style="min-height: fit-content;">
                            <InputIcon class="pi pi-lock"></InputIcon>
                            <Textarea v-model="input" class="w-full h-full pr-4" placeholder="发送加密信息"></Textarea>
                        </IconField>
                        <Button icon="pi pi-send" plain text></Button>
                    </div>
                </SplitterPanel>
            </Splitter>
        </div>
    </div>
</template>

<style scoped>
:deep(textarea) {
    resize: none;
    line-height: 1.5;
    vertical-align: middle;
}
</style>