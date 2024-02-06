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
                timestamp: '2024/2/6 19:54',
            },
            {
                text: '跑路跑路',
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
                timestamp: '2024/2/6 19:54',
            },
        ]
    },
]);
</script>

<template>
    <div class="card w-full flex justify-content-center h-full">
        <Listbox v-model="selectedMessage" :options="messages" optionLabel="label" optionGroupLabel="label"
            optionGroupChildren="items" class="w-full h-full">
            <template #optiongroup="slot">
                <div class="flex align-items-center">
                    <div>{{ slot.option.user.nickname }}</div>
                </div>
            </template>
            <template #option="slot">
                <!-- <div class="flex align-items-center"> -->
                <Button v-tooltip.bottom="slot.option.timestamp" plain text>{{ slot.option.text }}</Button>
                <!-- </div> -->
            </template>
        </Listbox>
    </div>
</template>


<style scoped>
.p-listbox {
    border: none;
}
</style>
