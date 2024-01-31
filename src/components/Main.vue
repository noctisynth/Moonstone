<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

// UI 控件
import SideToolBar from "./Main/SideToolBar.vue"
import Sessions from "./Main/Sessions.vue"
import EditorPanel from './Main/EditorPanel.vue';
import TopBar from "./Main/TopBar.vue";
</script>

<template>
    <Splitter style="height: 100vh" :pt="{
        gutter: { style: 'width: 3px; cursor: ew-resize' },
        gutterHandler: { style: 'display: none' }
    }">
        <SplitterPanel class="flex align-items-center justify-content-center fixed-width-panel" :size="15" :minSize="15"
            :maxSize="15">
            <SideToolBar />
        </SplitterPanel>
        <SplitterPanel class="flex align-items-center justify-content-center" :size="20">
            <Sessions />
        </SplitterPanel>
        <SplitterPanel :size="65">
            <Splitter layout="vertical" :pt="{
                gutter: { style: 'height: 3px; cursor: ns-resize' },
                gutterHandler: { style: 'display: none' }
            }">
                <SplitterPanel class="flex align-items-center justify-content-center" :size="10"> <TopBar /> </SplitterPanel>
                <SplitterPanel :size="75">
                    <!-- <Splitter :pt="{
                        gutter: { style: 'width: 3px; cursor: ew-resize' },
                        gutterHandler: { style: 'display: none' }
                    }">
                        <SplitterPanel class="flex align-items-center justify-content-center" :size="20"> Panel 3
                        </SplitterPanel>
                        <SplitterPanel class="flex align-items-center justify-content-center" :size="80">
                            cwerfre
                        </SplitterPanel>
                    </Splitter> -->
                </SplitterPanel>
                <SplitterPanel :size="15" style="overflow: hidden;">
                    <EditorPanel />
                </SplitterPanel>
            </Splitter>
        </SplitterPanel>
    </Splitter>
</template>

<style scoped>
.fixed-width-panel {
    flex: 0 0 10vw;
    min-width: 10vw;
    max-width: 10vw;
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
