<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

// UI 控件
import SideToolBar from "./Main/SideToolBar.vue"
import Sessions from "./Main/Sessions.vue"
import EditorPanel from './Main/EditorPanel.vue';
import TopBar from "./Main/TopBar.vue";
import Messages from "./Main/Messages.vue";

import { ref } from 'vue';

let session = ref()
let messages = ref([
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

function sessionChanged(node: any) {
    session.value = node;
}
</script>

<template>
    <Splitter class="h-full" style="border: none;" :pt="{
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
    </Splitter>
</template>

<style scoped>
.fixed-width-panel {
    /* flex: 0 0 3vw; */
    min-width: 8vw;
    max-width: 8vw;
}
</style>













<!-- <script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet2() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet2", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet2">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template> -->
