<script setup lang="ts">
import Vditor from 'vditor';
import { ref, onMounted, Ref } from 'vue';
import 'vditor/dist/index.css';
import { v4 as uuid4 } from 'uuid';

let messages: any = defineModel('messages')
const vditor: Ref<Vditor | undefined> = ref();


function onSubmit() {
    const value = vditor.value?.getValue();
    const trimed_value = value?.replace("\n", "").trim();

    if (trimed_value && trimed_value.length > 0) {
        const uuid = uuid4();

        let last_user_id = null;
        if (messages.value.length > 0)
            last_user_id = messages.value[messages.value.length - 1].user.id;

        if (last_user_id === "0") {
            messages.value[messages.value.length - 1].items.push({
                text: value,
                sequence: uuid,
                status: 'sending',
                timestamp: '2024/2/6 19:54',
            })
        } else {
            messages.value.push({
                user: {
                    id: '0',
                    nickname: '苏向夜',
                },
                timestamp: '2024/2/6 19:54',
                items: [
                    {
                        text: value,
                        sequence: uuid,
                        status: 'sending',
                        timestamp: '2024/2/6 19:54',
                    },
                ]
            });
        };
    };
    vditor.value?.setValue("", true);
};

function clear() {
    const value = vditor.value?.getValue().replace("\n", "").trim();
    if (!value)
        vditor.value?.setValue("")
}

onMounted(() => {
    vditor.value = new Vditor('vditor', {
        theme: 'dark',
        height: '100%',
        mode: 'ir',
        icon: 'material',
        cdn: '',
        cache: {
            enable: false,
        },
        toolbar: [
            'emoji', 'bold', 'italic', 'strike', 'link',
            '|',
            'line',
            'quote',
            'list',
            'ordered-list',
            'check',
            '|',
            'inline-code',
            'code',
        ],
        counter: {
            enable: true,
            max: 512,
        },
    });
});
</script>

<template>
    <div class="h-full text-white">
        <div id="vditor" @keydown.alt.enter.exact="onSubmit" @keyup.enter="clear"></div>
    </div>
</template>

<style scoped></style>