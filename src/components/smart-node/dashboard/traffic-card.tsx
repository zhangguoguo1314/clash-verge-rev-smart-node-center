import {
  Box,
  Skeleton,
  Typography,
  alpha,
  useTheme,
} from '@mui/material'
import { SpeedRounded } from '@mui/icons-material'
import { useTranslation } from 'react-i18next'

import { EnhancedCard } from '@/components/home/enhanced-card'
import { useDashboard } from '@/hooks/use-smart-node'

export const TrafficCard = () => {
  const { t } = useTranslation('smartNode')
  const theme = useTheme()
  const { dashboardData, isPending } = useDashboard()

  const isLoading = isPending

  return (
    <EnhancedCard
      title={t('dashboard.traffic')}
      icon={<SpeedRounded />}
      iconColor="secondary"
    >
      {isLoading ? (
        <Box>
          <Skeleton width="50%" height={24} />
          <Skeleton width="60%" height={24} />
        </Box>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1.5 }}>
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              p: 1.5,
              borderRadius: 1.5,
              bgcolor: alpha(
                theme.palette.secondary.main,
                0.06,
              ),
            }}
          >
            <Typography variant="body2" color="text.secondary">
              {t('dashboard.download')}
            </Typography>
            <Typography variant="h6" fontWeight="bold">
              {dashboardData?.download_speed?.toFixed(1) ??
                '0.0'}{' '}
              Mbps
            </Typography>
          </Box>
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              p: 1.5,
              borderRadius: 1.5,
              bgcolor: alpha(
                theme.palette.info.main,
                0.06,
              ),
            }}
          >
            <Typography variant="body2" color="text.secondary">
              {t('dashboard.upload')}
            </Typography>
            <Typography variant="h6" fontWeight="bold">
              {dashboardData?.upload_speed?.toFixed(1) ?? '0.0'}{' '}
              Mbps
            </Typography>
          </Box>
          <Box
            sx={{
              mt: 1,
              display: 'flex',
              justifyContent: 'space-between',
            }}
          >
            <Typography variant="caption" color="text.secondary">
              {t('dashboard.switchCount')}: {dashboardData?.switch_count ?? 0}
            </Typography>
            <Typography variant="caption" color="text.secondary">
              {t('dashboard.testCount')}: {dashboardData?.speed_test_count ?? 0}
            </Typography>
          </Box>
        </Box>
      )}
    </EnhancedCard>
  )
}
