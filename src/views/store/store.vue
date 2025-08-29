<script setup lang="ts">
import { useTemplateRef } from 'vue'
import { Setting } from '@element-plus/icons-vue'
import DataSource from './components/data-source.vue'
import DataFilter from '@/components/data-filter.vue'

type Command = 'source'
const handleCommand = (command: Command) => {
    console.log(command)
    if (command === 'source') {
        openDataSourceDialog()
    }
}

const dataSourceRef = useTemplateRef('dataSourceRef')
const openDataSourceDialog = () => {
    dataSourceRef.value?.open()
}
</script>

<template>
    <div class="store">
        <div class="store-header">
            <span>壁纸中心</span>
            <el-dropdown @command="handleCommand" class="store-setting">
                <el-link>
                    <el-icon size="20"><Setting /></el-icon>
                </el-link>
                <template #dropdown>
                    <el-dropdown-menu>
                        <el-dropdown-item command="source">设置壁纸源</el-dropdown-item>
                    </el-dropdown-menu>
                </template>
            </el-dropdown>
        </div>
        <div class="store-aside">
            <DataFilter />
        </div>
        <div class="store-content"></div>
    </div>
    <DataSource ref="dataSourceRef" />
</template>

<style lang="scss" scoped>
.store{
    min-height: 100%;
    display: grid;
    grid-template-rows: auto 1fr;      /* 第一行 header，高度自适应，第二行填满 */
    grid-template-columns: 240px 1fr;  /* aside 固定 240px，content 自适应 */
    grid-template-areas:
    "header header"
    "aside content";
    &-header{
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        grid-area: header;
        height: 4rem;
        padding: 0 2.4rem;
    }
    &-aside{
        grid-area: aside;
        background-color: beige;
        padding: 0.8rem 2rem;
    }
    &-content{
        grid-area: content;
        background-color: aliceblue;
        padding: 2rem;
    }

    &-setting {
        position: absolute;
        top: 50%;
        right: 2rem;
        transform: translateY(-50%);
    }
}

</style>
