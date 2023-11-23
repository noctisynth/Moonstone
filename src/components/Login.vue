
<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Description from "./Login/Description.vue";
import Login from "./Login/Login.vue";
</script>

<template>
    <Splitter style="height: 100vh">
        <SplitterPanel :size="35">
            <Description />
        </SplitterPanel>
        <SplitterPanel :size="65" class="flex align-items-center justify-content-center">
            <Login />
        </SplitterPanel>
    </Splitter>
</template>

<style scoped></style>













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
