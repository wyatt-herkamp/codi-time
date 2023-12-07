
<template>
    <ApiReference :configuration="configuration"/>
</template>

<script setup lang="ts">
import { apiURL } from '@/config';
import { ApiReference, type ReferenceConfiguration } from '@scalar/api-reference'

const configuration: ReferenceConfiguration = {
    spec: {
        url: `${apiURL}api/docs/openapi.json`,
    }
}
</script>
