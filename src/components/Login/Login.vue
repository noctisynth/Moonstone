
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { useField, useForm } from 'vee-validate';
import { ref } from "vue";
import { useRouter } from 'vue-router';

const router = useRouter();
const { handleSubmit, resetForm: _resetForm } = useForm();

const { value: server, errorMessage: invalidServer } = useField('server', validateServer);
const { value: identity, errorMessage: invalidUsername } = useField('username', validateUsername);
const { value: password, errorMessage: invalidPassword } = useField('password', validatePassword);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);
const info = ref("");
const error = ref("");

function validateServer(value: string) {
    if (!value) {
        return "请输入服务端地址！";
    }

    return true;
}

function validateUsername(value: string) {
    if (!value) {
        return "请输入用户名！";
    }

    return true;
}

function validatePassword(value: string) {
    console.log()
    if (!value) {
        return "请输入密钥！";
    }

    return true;
}

function validateAccept(value: boolean) {
    if (!value) {
        return "请阅读并同意浊莲文明协定！";
    }

    return true;
}

const login = handleSubmit(async () => {
    info.value = "登录中...";

    if (!accept.value) {
        invalidAccept.value = "请先阅读并同意浊莲文明协定！";
        return
    }

    let callback: { status: boolean; session_key: string; error: string } = JSON.parse(
        await invoke("login_handler", { server: server.value, identity: identity.value, password: password.value })
    );
    if (callback["status"]) {
        localStorage.setItem("server", server.value);
        localStorage.setItem("isLoggedIn", "true");
        localStorage.setItem("session_key", callback["session_key"]);
        info.value = "登录成功";
        router.push('/');
    } else {
        info.value = "";
        error.value = callback["error"];
    }
})
</script>

<template>
    <div class="card flex justify-content-center p-fluid">
        <div v-focustrap class="w-full" style="max-width: 100%">
            <div class="filed">
                <h1>Moonstone</h1>
            </div>
            <form @submit="login" class="flex flex-column gap-2">
                <div class="field">
                    <span class="p-float-label p-input-icon-right">
                        <i class="pi pi-globe"></i>
                        <InputText id="server-input" name="server" v-model="server" type="text"
                            :class="{ 'p-invalid': invalidServer }" aria-describedby="server-error" />
                        <label for="server-input">Server</label>
                    </span>
                    <small class="p-error" id="username-error">{{ invalidServer || null }}</small>
                </div>
                <div class="field">
                    <span class="p-float-label p-input-icon-right">
                        <i class="pi pi-user"></i>
                        <InputText id="username-input" name="username" v-model="identity" type="text"
                            autocomplete="username email" :class="{ 'p-invalid': invalidUsername }"
                            aria-describedby="username-error" />
                        <label for="username-input">Identity</label>
                    </span>
                    <small class="p-error" id="username-error">{{ invalidUsername || null }}</small>
                </div>
                <div class="field">
                    <span class="p-float-label">
                        <Password v-model="password" type="text" inputId="password-input"
                            :class="{ 'p-invalid': invalidPassword }" :feedback="false" aria-describedby="password-error"
                            toggleMask />
                        <label for="password-input">Password</label>
                    </span>
                    <small class="p-error" id="password-error">{{ invalidPassword || null }}</small>
                </div>
                <div class="field-checkbox">
                    <Checkbox v-model="accept" :binary="true" name="accept" inputId="accept-input"
                        :class="{ 'p-invalid': invalidAccept }" aria-describedby="accept-error" />
                    <label for="accept-input">我已阅读并同意浊莲文明协定</label>
                </div>
                <small class="p-error" id="accept-error">{{ invalidAccept || null }}</small>
                <small class="p-info" id="text-info">{{ info || null }}</small>
                <small class="p-error" id="text-error">{{ error || null }}</small>
                <Button type="submit" label="登录" class="mt-2"></Button>
            </form>
        </div>
    </div>
</template>

<style scoped></style>
