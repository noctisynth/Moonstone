<script setup lang="ts">
import { useToast } from 'primevue/usetoast';
import { useField, useForm } from 'vee-validate';
import Editor from 'primevue/editor';

const { handleSubmit, resetForm } = useForm();
const { value: message, errorMessage: _ } = useField('value', validateField);
const toast = useToast();

function validateField(value: any) {
    if (!value) {
        return 'Content is required.';
    }

    return true;
}

const onSubmit = handleSubmit((values) => {
    if (values.value && values.value.length > 0) {
        toast.add({ severity: 'info', summary: 'Blog Submitted', detail: 'The blog is uploaded', life: 3000 });
        resetForm();
    }
});
</script>

<template>
    <form @submit="onSubmit" class="h-full">
        <Editor v-model="message" style="height: calc(100% - 42px);">
            <template v-slot:toolbar>
                <span class="ql-formats flex">
                    <div style="margin-right: auto;">
                        <button class="ql-bold"></button>
                        <button class="ql-italic"></button>
                        <button class="ql-underline"></button>
                    </div>
                    <Button class="flex justify-content-center text-white w-auto">
                        <i class="pi pi-send"></i>
                    </Button>
                </span>

            </template>
        </Editor>
    </form>
</template>

<style scoped></style>