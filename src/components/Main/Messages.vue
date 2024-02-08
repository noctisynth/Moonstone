<script setup lang="ts">
import { ref } from "vue";
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();
const selectedMessage = ref();
let messages: any = defineModel()

function render(text: string) {
    if (text.split("\n\n").length <= 1) return text
    else return md.render(text)
}
</script>

<template>
    <div class="card w-full flex justify-content-center h-full">
        <Listbox v-model="selectedMessage" :options="messages" optionLabel="text" optionValue="sequence"
            optionGroupLabel="user.nickname" optionGroupChildren="items" class="w-full h-full">
            <template #empty>
                <p></p>
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
</template>


<style scoped>
.p-listbox {
    border: none;
}
</style>
