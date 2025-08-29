import { ref } from 'vue';

export const useLoading = (delay: number = 100) => {
    const loading = ref(false);
    let loadingStartTime: number = 0;
    const startLoading = () => {
        loading.value = true;
        loadingStartTime = new Date().getTime();
    };
    const endLoading = () => {
        const now = new Date().getTime();
        const diff = now - loadingStartTime;
        if (diff < delay) {
            setTimeout(() => {
                loading.value = false;
            }, delay - diff);
        } else {
            loading.value = false;
        }
    };
    return {
        loading,
        startLoading,
        endLoading,
    };
};

export default useLoading;
