import { Hero as BaseHero } from '@/components/Hero'
import { HeroScreenshot } from '@/components/HeroScreenshot'

export function Hero({ page }) {
  return (
    <BaseHero page={page} subDescription="" primaryCta={undefined}>
      <HeroScreenshot
        src="/assets/splash.png"
        alt="A screenshot of a lighthouse"
        width={1392}
        height={860}
      />
    </BaseHero>
  )
}
