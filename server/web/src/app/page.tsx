// import ComponentHome from '@timex/components/page/home'
import dynamic from 'next/dynamic'

const ComponentHome = dynamic(() => import('@timex/components/page/home'), {
  loading: () => <p>Loading...</p>,
})

export default function Home() {
  return <ComponentHome />
}