
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
