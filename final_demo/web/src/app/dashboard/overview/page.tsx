import { getOverview } from "@/api/overview";
import DashboardOverview from "@/components/dashboard/overview";
import { Flex } from "@chakra-ui/react";
import { QueryClient } from "@tanstack/react-query";

interface PageProps {
  searchParams: {
    polyclinic: string;
    dateFrom: string;
    dateTo: string;
  }
}

export default function OverviewPage({ searchParams }: PageProps) {
  return (
    <Flex w={'full'} direction={"column"} flex={1} p={8}>
      <DashboardOverview />
    </Flex>
  )
}