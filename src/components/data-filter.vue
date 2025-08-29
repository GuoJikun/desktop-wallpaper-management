<script setup lang="ts">
import { ref } from 'vue'

const dataSourceList = [
    { value: 'jsr', label: 'jsr' },
    { value: 'npm', label: 'npm' },
    { value: 'github', label: 'github' },
    { value: 'gitee', label: 'gitee' },
]

const typeList = [
    { value: 'html', label: '网页' },
    { value: 'image', label: '图片' },
    { value: 'video', label: '视频' },
]

const labelList = [
    { value: '风景', label: '风景' },
    { value: '动物', label: '动物' },
    { value: '明星', label: '明星' },
]

interface IFormItem {
    dataSource: string[]
    type: string[]
    label: string[]
}
const formItem = ref<IFormItem>({
    dataSource: [],
    type: [],
    label: [],
})

const emit = defineEmits<{
    change: [val: IFormItem]
}>()
const handleChange = (): void => {
    console.log(formItem.value)
    emit('change', formItem.value)
}
</script>

<template>
    <div>
        <div class="filter-item">
            <div class="filter-item-header">
                <span>数据源</span>
            </div>
            <div class="filter-item-body">
                <el-checkbox-group v-model="formItem.dataSource" @change="handleChange">
                    <div style="display: flex; flex-direction: column">
                        <el-checkbox v-for="item in dataSourceList" :key="item.value" :value="item.value">{{
                            item.label
                        }}</el-checkbox>
                    </div>
                </el-checkbox-group>
            </div>
        </div>
        <div class="filter-item">
            <div class="filter-item-header">
                <span>类型</span>
            </div>
            <div class="filter-item-body">
                <el-checkbox-group v-model="formItem.type" @change="handleChange">
                    <div style="display: flex; flex-direction: column">
                        <el-checkbox v-for="item in typeList" :key="item.value" :value="item.value">{{
                            item.label
                        }}</el-checkbox>
                    </div>
                </el-checkbox-group>
            </div>
        </div>
        <div class="filter-item">
            <div class="filter-item-header">
                <span>标签</span>
            </div>
            <div class="filter-item-body">
                <el-checkbox-group v-model="formItem.label" @change="handleChange">
                    <div style="display: flex; flex-direction: column">
                        <el-checkbox v-for="item in labelList" :key="item.value" :value="item.value">{{
                            item.label
                        }}</el-checkbox>
                    </div>
                </el-checkbox-group>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.filter {
    &-item {
        --el-checkbox-height: 2.4rem;
        &:not(:last-child) {
            margin-bottom: 1.2rem;
        }

        &-header {
            margin-bottom: 0.8rem;
        }

        &-body {
            padding-left: 1.2rem;
        }
    }
}
</style>
