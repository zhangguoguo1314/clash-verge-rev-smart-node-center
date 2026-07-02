import { Grid, Skeleton } from '@mui/material'
import { Suspense, lazy } from 'react'
import { useTranslation } from 'react-i18next'

const LazyInfoCard = lazy(() =>
  import('@/components/smart-node/dashboard/info-card').then(
    (m) => ({ default: m.InfoCard }),
  ),
)
const LazyTrafficCard = lazy(() =>
  import(
    '@/components/smart-node/dashboard/traffic-card'
  ).then((m) => ({ default: m.TrafficCard })),
)
const LazySystemCard = lazy(() =>
  import(
    '@/components/smart-node/dashboard/system-card'
  ).then((m) => ({ default: m.SystemCard })),
)

const cardSkeleton = (
  <Skeleton variant="rectangular" height={180} />
)

export const SmartNodeDashboard = () => {
  return (
    <Grid container spacing={1.5} columns={{ xs: 6, sm: 6, md: 12 }}>
      <Grid size={{ xs: 6, md: 4 }}>
        <Suspense fallback={cardSkeleton}>
          <LazyInfoCard />
        </Suspense>
      </Grid>
      <Grid size={{ xs: 6, md: 4 }}>
        <Suspense fallback={cardSkeleton}>
          <LazyTrafficCard />
        </Suspense>
      </Grid>
      <Grid size={{ xs: 6, md: 4 }}>
        <Suspense fallback={cardSkeleton}>
          <LazySystemCard />
        </Suspense>
      </Grid>
    </Grid>
  )
}
