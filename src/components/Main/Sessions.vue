<script setup lang="ts">
import { ref } from 'vue';

const sessions = ref([
    {
        key: '0',
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
        key: '1',
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
        key: '2',
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

</script>
  
<template>
    <div class="card flex justify-content-center h-full">
        <Tree :value="sessions" :filter="true" filterMode="lenient" class="w-full"></Tree>
    </div>
</template>

<style scoped></style>