"use client"

import { Flex } from "@chakra-ui/react";
import OverviewHeader from "./header";

export default function DashboardOverview() {
  return (
    <Flex direction={"column"} gap={10}>
      <OverviewHeader />
      {/* <OverviewStats />
      <Flex w={'full'} gap={5}>
        <DoughtnutChart />
        <DoughtnutChart />
      </Flex> */}
    </Flex>
  )
}