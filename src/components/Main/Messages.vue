<script setup lang="ts">
import { ref } from "vue";

const selectedMessage = ref();
const messages = ref([
    {
        user: {
            nickname: '雪花'
        },
        sequence: '',
        timestamp: '2024/2/6 19:54',
        items: [
            {
                text: '我要跑路',
                status: 'verified',
                timestamp: '2024/2/6 19:54',
            },
            {
                text: '跑路跑路',
                status: 'verified',
                timestamp: '2024/2/6 19:54',
            },
        ]
    },
    {
        user: {
            nickname: '苏向夜'
        },
        sequence: '',
        items: [
            {
                text: '不许跑',
                status: 'sending',
                timestamp: '2024/2/6 19:54',
            },
            {
                text: '#敲',
                status: 'failed',
                timestamp: '2024/2/6 19:54',
            },
            {
                text: '#捉',
                status: 'reached',
                timestamp: '2024/2/6 19:54',
            },
            {
                text: '谴责！',
                status: 'check',
                timestamp: '2024/2/6 19:54',
            },
        ]
    },
]);
</script>

<template>
    <div class="card w-full flex justify-content-center h-full">
        <Listbox v-model="selectedMessage" :options="messages" optionGroupLabel=" " optionGroupChildren="items"
            class="w-full h-full">
            <template #optiongroup="slot">
                <div class="flex align-items-center">
                    <div>{{ slot.option.user.nickname }}</div>
                </div>
            </template>
            <template #option="slot">
                <div v-tooltip.bottom="slot.option.timestamp">
                    <div v-if="slot.option.status === 'sending'">
                        <Button class="text-white" :label="slot.option.text" badge=" "
                            badgeClass="bg-primary flex justify-content-center align-items-center pi pi-spin pi-spinner"
                            plain text>
                        </Button>
                    </div>
                    <div v-else-if="slot.option.status === 'failed'">
                        <Button class="text-white" :label="slot.option.text" badge=" "
                            badgeClass="bg-red-900 flex justify-content-center align-items-center pi pi-times" plain text>
                        </Button>
                    </div>
                    <div v-else-if="slot.option.status === 'verified'">
                        <Button class="text-white" :label="slot.option.text" badge=" "
                            badgeClass="bg-green-900 flex justify-content-center align-items-center pi pi-verified" plain
                            text>
                        </Button>
                    </div>
                    <div v-else-if="slot.option.status === 'check'">
                        <Button class="text-white" :label="slot.option.text" badge=" "
                            badgeClass="bg-green-900 flex justify-content-center align-items-center pi pi-check" plain text>
                        </Button>
                    </div>
                    <div v-else>
                        <Button class="text-white" :label="slot.option.text" plain text>
                        </Button>
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
