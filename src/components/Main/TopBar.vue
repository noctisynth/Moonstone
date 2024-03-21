<script setup lang="ts">
import Button from 'primevue/button';
import Toolbar from 'primevue/toolbar';

const session: any = defineModel()
</script>

<template>
    <div class="card w-full">
        <Toolbar style="border: none; padding: 1rem 1rem 1rem 1.5rem">
            <template #start>
                {{ session.label }}
            </template>

            <template #end>
                <Button icon="pi pi-phone" class="mr-2" text plain></Button>
                <Button icon="pi pi-video" class="mr-2" text plain></Button>
                <Button icon="pi pi-envelope" class="mr-2" text plain></Button>
                <Button icon="pi pi-ellipsis-h" text plain></Button>
            </template>
        </Toolbar>
    </div>
</template>

<style scoped></style>