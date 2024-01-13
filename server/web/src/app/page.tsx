'use client'
// import ComponentHome from '@timex/components/page/home'
import axios from 'axios'
import dynamic from 'next/dynamic'
import { useEffect } from 'react'

const ComponentHome = dynamic(() => import('@timex/components/page/home'), {
  loading: () => <p>Loading...</p>,
})

export default function Home() {
  return <ComponentHome />
}