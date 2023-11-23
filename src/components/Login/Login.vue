
<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { ref } from 'vue';
const value = ref(null);
const accept = ref(false);
</script>

<template>
    <div class="card flex justify-content-center p-fluid">
        <div v-focustrap class="w-full" style="max-width: 100%">
            <div class="filed">
                <h1>MoonStone</h1>
            </div>
            <form>
                <div class="field">
                    <span class="p-float-label p-input-icon-right">
                        <i class="pi pi-user"></i>
                        <InputText id="username" v-model="value" />
                        <label for="username">Username</label>
                    </span>
                </div>
                <div class="field">
                    <span class="p-float-label">
                        <Password v-model="value" :feedback="false" toggleMask></Password>
                        <label for="password">Password</label>
                    </span>
                </div>
                <div class="field-checkbox">
                    <Checkbox id="accept" v-model="accept" name="accept" value="Accept" />
                    <label for="accept">我已阅读并同意 Noctisynth 文明协定</label>
                </div>
                <Button type="submit" label="Submit" class="mt-2"></Button>
            </form>
        </div>
    </div>
</template>

<style scoped></style>
