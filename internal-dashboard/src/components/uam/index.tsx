"use client";

import { useMemo, useState } from "react";
import { useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import { IoMdRefresh } from "react-icons/io";
import { useQuery } from "@tanstack/react-query";
import { getUam } from "@/libs/api/uam";

import UAMTable from "@/components/uam/table";
import Pagination from "@/components/pagination";
import Search from "@/components/search";
import { UAMLoading } from "@/components/uam/loading";
import { FilterMenu, FilterTags } from "./filter";

export default function UAMData() {
  const params = useSearchParams();
  const page = params.get('page') || "1";
  const limit = params.get('limit') || "10";

  const [searchTerm, setSearchTerm] = useState<string>("");
  const [selectedStatus, setSelectedStatus] = useState<string>("all");

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    setSearchTerm(value);
  };

  const onClearSearch = () => setSearchTerm('');

  const {
    data: datas,
    isLoading,
    refetch: refetchUAM
  } = useQuery({
    queryKey: ['uam', { page, limit }],
    queryFn: () => getUam({ page, limit }),
    refetchOnWindowFocus: false,
    refetchInterval: 5 * 60 * 1000 // 5 minutes
  });

  const filteredData = useMemo(() => {
    if (!datas) return [];

    let filtered = datas.data;

    if (searchTerm) {
      filtered = filtered.filter(item =>
        item.fullName.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    if (selectedStatus !== "all") {
      filtered = filtered.filter(item => item.verification === selectedStatus);
    }

    return filtered;
  }, [searchTerm, selectedStatus, datas]);

  const onClearFilter = () => setSelectedStatus("all");

  if (isLoading) return <UAMLoading />;
  if (datas?.data.length === 0) return <Flex>No Data.</Flex>;
  if (!datas) return null;

  return (
    <Flex direction={'column'} gap={8}>
      <Flex gap={3}>
        <Search
          defaultValue={searchTerm}
          onFillSearch={onFillSearch}
          onClearSearch={onClearSearch}
        />
        <FilterMenu
          selectedStatus={selectedStatus}
          setSelectedStatus={setSelectedStatus}
        />
        <Button
          size={'lg'}
          rounded={'xl'}
          p={2}
          onClick={() => refetchUAM()}
        >
          <Icon as={IoMdRefresh} boxSize={5} />
        </Button>
      </Flex>

      <Flex direction={'column'} gap={4}>
        {selectedStatus !== "all" && (
          <FilterTags
            selectedStatus={selectedStatus}
            onClearFilter={onClearFilter}
          />
        )}

        {filteredData.length === 0
          ? (
            <Text fontSize={"md"} color={'neutral.700'} textAlign={'center'}>
              No data found.
            </Text>
          )
          : (
            <Flex direction={'column'} gap={8} zIndex={0}>
              <UAMTable datas={filteredData} />
              <Pagination totalPages={datas.pagination.totalPage} />
            </Flex>
          )
        }
      </Flex>
    </Flex>
  )
}