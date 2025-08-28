<script setup lang="ts">
interface ICardProps {
    title: string
    src: string
    description: string
}

const props = defineProps<ICardProps>()
</script>

<template>
    <div class="card">
        <div>
            <img :src="props.src" alt="" />
        </div>
        <div class="card-extra">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped lang="scss">
.card {
    border: 1px solid #eee;
    border-radius: 8px;
    background-color: #fff;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    position: relative;
    width: 200px;
    height: 200px;
    &-extra {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        background: rgba(255, 255, 255, 0.8);
        padding: 8px;
        border-top: 1px solid #eee;
    }

    &:hover {
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        transform: translateY(-100%);
    }
}
</style>
