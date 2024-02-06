<script setup lang="ts">
import { ref } from "vue";

const selectedMessage = ref();
const groupedMessages = ref([
    {
        label: '雪花',
        items: [
            { label: '我要跑路！', value: 'Berlin' },
            { label: '跑路跑路', value: 'Frankfurt' },
        ]
    },
    {
        label: '苏向夜',
        code: 'US',
        items: [
            { label: '不许跑', value: 'Chicago' },
        ]
    },
    {
        label: '雪花',
        code: 'JP',
        items: [
            { label: '就要跑', value: 'Kyoto' },
        ]
    },
    {
        label: '苏向夜',
        code: 'JP',
        items: [
            { label: '谴责！', value: 'fre' },
        ]
    },
    {
        label: '雪花',
        code: 'JP',
        items: [
            { label: '不许谴责！', value: 'Kyoto' },
        ]
    },
    {
        label: '苏向夜',
        code: 'JP',
        items: [
            { label: '谴责！', value: 'Kyoto' },
        ]
    },
]);
</script>

<template>
    <div class="card w-full flex justify-content-center">
        <Listbox v-model="selectedMessage" :options="groupedMessages" optionLabel="label" optionGroupLabel="label"
            optionGroupChildren="items" class="w-full h-full" listStyle="max-height: 480px">
            <!-- <template #optiongroup="slotProps">
                <div class="flex align-items-center">
                    <div>{{ slotProps.option.label }}</div>
                </div>
            </template> -->
        </Listbox>
    </div>
</template>


<style scoped>
.p-listbox {
    border: none;
}
</style>
