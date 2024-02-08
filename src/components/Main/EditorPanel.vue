<script setup lang="ts">
import Vditor from 'vditor';
import { ref, onMounted, Ref } from 'vue';
import 'vditor/dist/index.css';

const vditor: Ref<Vditor | undefined> = ref();


onMounted(() => {
    vditor.value = new Vditor('vditor', {
        theme: 'dark',
        height: '100%',
        mode: 'ir',
        icon: 'material',
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

const session: any = defineModel('session')
let messages: any = defineModel('messages')


function onSubmit() {
    const value = vditor.value?.getValue();
    const trimed_value = value?.replace("\n", "").trim();
    vditor.value?.setValue("", true);

    if (trimed_value && trimed_value.length > 0) {
        // console.log(messages.value[messages.value.length - 1])
        // console.log(messages.value[1])
        messages.value.push({
            user: {
                nickname: '苏向夜',
            },
            sequence: 'xxxx',
            timestamp: '2024/2/6 19:54',
            items: [
                {
                    text: value,
                    status: 'sending',
                    timestamp: '2024/2/6 19:54',
                },
            ]
        });
        console.log(session.value === null);
    }
};

function clear() {
    const value = vditor.value?.getValue().replace("\n", "").trim();
    console.log(value === "")
    if (!value)
        vditor.value?.setValue("")
}
</script>

<template>
    <div class="h-full text-white">
        <div id="vditor" @keyup.alt.enter.exact="onSubmit" @keyup.enter="clear"></div>
    </div>
</template>

<style scoped></style>