<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';

import SideToolBar from "./components/SideToolBar.vue"
</script>

<template>
    <Splitter style="height: 100vh">
        <SplitterPanel class="flex align-items-center justify-content-center fixed-width-panel" :size="15" :minSize="15"
            :maxSize="15">
            <SideToolBar />
        </SplitterPanel>
        <SplitterPanel class="flex align-items-center justify-content-center" :size="20"> Panel 2 </SplitterPanel>
        <SplitterPanel :size="65">
            <Splitter layout="vertical">
                <SplitterPanel class="flex align-items-center justify-content-center" :size="15"> Panel 2 </SplitterPanel>
                <SplitterPanel :size="85">
                    <Splitter>
                        <SplitterPanel class="flex align-items-center justify-content-center" :size="20"> Panel 3
                        </SplitterPanel>
                        <SplitterPanel class="flex align-items-center justify-content-center" :size="80"> Panel 4
                        </SplitterPanel>
                    </Splitter>
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
