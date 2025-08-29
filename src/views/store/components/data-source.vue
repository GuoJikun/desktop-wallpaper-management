<script setup lang="ts">
import { ref } from 'vue'
import {useDialog} from '@/composables/dialog.ts'

const {visible, handleClose, handleOpen} = useDialog()

const tableData = ref([])

defineExpose({
    open: handleOpen
})
</script>

<template>
    <el-dialog v-model="visible" title="壁纸源">

    </el-dialog>
</template>

<style scoped></style>
