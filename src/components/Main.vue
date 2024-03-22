<script setup lang="ts">
// import SideToolBar from "./Main/SideToolBar.vue"
// import Sessions from "./Main/Sessions.vue"
// import EditorPanel from './Main/EditorPanel.vue';
// import TopBar from "./Main/TopBar.vue";
// import Messages from "./Main/Messages.vue";
// import Splitter from "primevue/splitter";
// import SplitterPanel from "primevue/splitterpanel";

// import { ref } from 'vue';

// let session = ref()
// let messages = ref([
//     {
//         user: {
//             nickname: '雪花'
//         },
//         sequence: '',
//         timestamp: '2024/2/6 19:54',
//         items: [
//             {
//                 text: '我要跑路',
//                 status: 'verified',
//                 timestamp: '2024/2/6 19:54',
//             },
//             {
//                 text: '跑路跑路',
//                 status: 'verified',
//                 timestamp: '2024/2/6 19:54',
//             },
//         ]
//     },
//     {
//         user: {
//             nickname: '苏向夜'
//         },
//         sequence: '',
//         items: [
//             {
//                 text: '不许跑',
//                 status: 'sending',
//                 timestamp: '2024/2/6 19:54',
//             },
//             {
//                 text: '#敲',
//                 status: 'failed',
//                 timestamp: '2024/2/6 19:54',
//             },
//             {
//                 text: '#捉',
//                 status: 'reached',
//                 timestamp: '2024/2/6 19:54',
//             },
//             {
//                 text: '谴责！',
//                 status: 'check',
//                 timestamp: '2024/2/6 19:54',
//             },
//         ]
//     },
// ]);

// function sessionChanged(node: any) {
//     session.value = node;
// }
</script>

<template>
    <!-- <Splitter class="h-full" style="border: none;" :pt="{
        gutter: { style: 'width: 3px; cursor: ew-resize' },
        gutterHandler: { style: 'display: none' }
    }">
        <SplitterPanel class="flex align-items-center justify-content-center fixed-width-panel" :size="15" :minSize="15"
            :maxSize="15">
            <SideToolBar />
        </SplitterPanel>
        <SplitterPanel class="flex align-items-center justify-content-center" :size="20">
            <Sessions @session-changed="sessionChanged" />
        </SplitterPanel>
        <SplitterPanel :size="65">
            <Splitter v-if="session" layout="vertical" :pt="{
        gutter: { style: 'height: 3px; cursor: ns-resize' },
        gutterHandler: { style: 'display: none' }
    }">
                <SplitterPanel class="flex align-items-center justify-content-center" :size="10">
                    <TopBar v-model="session" />
                </SplitterPanel>
                <SplitterPanel :size="65" style="overflow-y: auto;">
                    <Messages v-model="messages" />
                </SplitterPanel>
                <SplitterPanel :size="25" style="overflow: hidden;">
                    <EditorPanel v-model:session="session" v-model:messages="messages" />
                </SplitterPanel>
            </Splitter>
        </SplitterPanel>
    </Splitter> -->
</template>

<style scoped></style>
