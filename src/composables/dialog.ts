import { ref } from 'vue'

export const useDialog = () => {
    const visible = ref(false);
    const loading = ref(false);

    const handleClose = () => {
        visible.value = false;
    }

    const handleOpen = () => {
        visible.value = true;
    }

    return {
        visible,
        loading,
        handleClose,
        handleOpen,
    }
}