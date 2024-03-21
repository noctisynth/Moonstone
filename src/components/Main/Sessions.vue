<script setup lang="ts">
import Tree from 'primevue/tree';
import { ref } from 'vue';

const emit = defineEmits(['sessionChanged'])

const sessions = ref([
    {
        key: 'tunnel',
        label: '信道',
        data: '信道列表',
        icon: 'pi pi-fw pi-inbox',
        children: [
            {
                key: '0-0',
                label: '3691 信道',
                icon: 'pi pi-fw pi-sign-in',
            },
        ],
    },
    {
        key: 'user',
        label: '联系人',
        data: '联系人',
        icon: 'pi pi-fw pi-users',
        children: [
            {
                key: '1-0',
                label: '雪花',
                icon: 'pi pi-fw pi-user',
            },
            {
                key: '1-1',
                label: '是源道呀',
                icon: 'pi pi-fw pi-user',
            },
        ],
    },
    {
        key: 'room',
        label: '房间',
        icon: 'pi pi-fw pi-home',
        children: [
            {
                key: '1-0',
                label: 'Noctisynth',
                icon: 'pi pi-fw pi-user',
            },
        ],
    },
]);

const onSessionSelect = (node: any) => {
    if (!["tunnel", "user", "room"].includes(node.key)) {
        emit("sessionChanged", node);
    }
};
</script>

<template>
    <div class="card flex justify-content-center h-full w-full">
        <Tree id="sessions" style="border: none;" :filter="true" filterMode="lenient" :value="sessions"
            selectionMode="single" @nodeSelect="onSessionSelect" class="w-full">
        </Tree>
    </div>
</template>

<style scoped></style>