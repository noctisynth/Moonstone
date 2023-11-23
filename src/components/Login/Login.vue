
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { useField, useForm } from 'vee-validate';
import { useRouter } from 'vue-router'; // 导入 Vue Router 的 useRouter

const router = useRouter(); // 获取路由实例
const { handleSubmit, resetForm: _resetForm } = useForm();

const { value: username, errorMessage: invalidUsername } = useField('username', validateUsername);
const { value: password, errorMessage: invalidPassword } = useField('password', validatePassword);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);

function validateUsername(value: any) {
    if (!value) {
        return "请输入用户名！";
    }

    return true;
}


function validatePassword(value: any) {
    if (!value) {
        return "请输入密钥！";
    }

    return true;
}


function validateAccept(value: any) {
    if (!value) {
        return "请阅读并同意 Noctisynth 文明协定！";
    }

    return true;
}


const login = handleSubmit(async () => {
    if (!accept) {
        invalidAccept.value = "请先阅读并同意 Noctisynth 文明协定！";
        return
    }
    if (await invoke("login", { username: username.value, password: password.value })) {
        console.log("登录成功!");
        localStorage.setItem("isLoggedIn", "true");
        router.push('/');
    }
})
</script>

<template>
    <div class="card flex justify-content-center p-fluid">
        <div v-focustrap class="w-full" style="max-width: 100%">
            <div class="filed">
                <h1>MoonStone</h1>
            </div>
            <form @submit="login" class="flex flex-column gap-2">
                <div class="field">
                    <span class="p-float-label p-input-icon-right">
                        <i class="pi pi-user"></i>
                        <InputText id="username" name="username" v-model="username" type="text"
                            :class="{ 'p-invalid': invalidUsername }" aria-describedby="text-error" />
                        <label for="username">Username</label>
                    </span>
                    <small class="p-error" id="text-error">{{ invalidUsername || null }}</small>
                </div>
                <div class="field">
                    <span class="p-float-label">
                        <Password id="password" v-model="password" type="text" :class="{ 'p-invalid': invalidPassword }"
                            :feedback="false" aria-describedby="text-error" toggleMask />
                        <label for="password">Password</label>
                    </span>
                    <small class="p-error" id="text-error">{{ invalidPassword || null }}</small>
                </div>
                <div class="field-checkbox">
                    <Checkbox id="accept" v-model="accept" name="accept" value="Accept"
                        :class="{ 'p-invalid': invalidPassword }" aria-describedby="text-error" />
                    <label for="accept">我已阅读并同意 Noctisynth 文明协定</label>
                </div>
                <small class="p-error" id="text-error">{{ invalidAccept || null }}</small>
                <Button type="submit" label="登录" class="mt-2"></Button>
            </form>
        </div>
    </div>
</template>

<style scoped></style>
