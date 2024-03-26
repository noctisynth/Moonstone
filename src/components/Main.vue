<script setup lang="ts">
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Sidebar from 'primevue/sidebar';
import { ref } from 'vue';

const emit = defineEmits(['onClose'])

defineModel('session');
const mobile = defineModel('mobile');

if (mobile.value === null) {
    mobile.value = true
}

const showSidebar = ref<boolean>(false)
</script>

<template>
    <div class="w-full h-full">
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
        <div class="h-full w-full">
            <div class="flex justify-content-center align-items-center">
                <div>请等待更新</div>
            </div>
        </div>
    </div>
</template>

<style scoped></style>