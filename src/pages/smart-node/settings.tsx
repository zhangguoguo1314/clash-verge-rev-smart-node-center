import { Stack } from '@mui/material'
import { useTranslation } from 'react-i18next'

import { GeneralSettings } from '@/components/smart-node/settings/general-settings'
import { SpeedSettings } from '@/components/smart-node/settings/speed-settings'

export const SmartNodeSettings = () => {
  const { t } = useTranslation('smartNode')

  return (
    <Stack spacing={3}>
      <GeneralSettings />
      <SpeedSettings />
    </Stack>
  )
}
