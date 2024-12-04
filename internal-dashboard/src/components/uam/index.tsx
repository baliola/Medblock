"use client";

import { useEffect, useState } from "react";
// import { useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import { IoMdRefresh } from "react-icons/io";

import UAMTable from "@/components/uam/table";
// import Pagination from "@/components/pagination";
import Search from "@/components/search";
import { UAMLoading } from "@/components/uam/loading";
import { FilterMenu, FilterTags } from "./filter";
import { usePatientStore } from "@/store/patients.store";
import { PatientWithNik } from "@/canister/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { useUserPrincipal } from "@ic-reactor/react";
import { uamHeader } from "@/constants/contents/uam/header";
import UAMDetail from "./detail";

const PatientList = ({ user }: { user: string | null }) => {
  // const params = useSearchParams();
  // const page = params.get('page') || "1";
  // const limit = params.get('limit') || "10";
  const { patients, setPatients } = usePatientStore();
  const principal = useUserPrincipal()

  const { call: getPatientList, loading: loadingGetPatientList } = usePatientQuery({
    functionName: "get_patient_list_admin",
    refetchOnMount: true,
    onSuccess(data) {
      const result = data?.patients ?? [];
      setPatients(result);
    },
    onError(error) {
      setPatients([])
      console.error(error);
    },
  });
  
  const [searchTerm, setSearchTerm] = useState<string>("");
  const [selectedStatus, setSelectedStatus] = useState<string>("all");

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    setSearchTerm(value);
  };

  const onClearSearch = () => setSearchTerm('');
  const onClearFilter = () => setSelectedStatus("all");

  const filteredData = (datas: Array<PatientWithNik>) => {
    if (!datas) return [];

    let filtered = datas;

    if (searchTerm) {
      filtered = filtered.filter(item =>
        item.info.V1.name.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    if (selectedStatus !== "all") {
      filtered = filtered.filter(item => Object.keys(item.info.V1.kyc_status)[0] === selectedStatus);
    }

    return filtered;
  }

  // const {
  //   data: datas,
  //   isLoading,
  //   refetch: refetchUAM
  // } = useQuery({
  //   queryKey: ['uam', { page, limit }],
  //   queryFn: () => getUam({ page, limit }),
  //   refetchOnWindowFocus: false,
  //   refetchInterval: 5 * 60 * 1000 // 5 minutes
  // });

  // const filteredData = useMemo(() => {
  //   if (!datas) return [];

  //   let filtered = datas.data;

  //   if (searchTerm) {
  //     filtered = filtered.filter(item =>
  //       item.fullName.toLowerCase().includes(searchTerm.toLowerCase())
  //     );
  //   }

  //   if (selectedStatus !== "all") {
  //     filtered = filtered.filter(item => item.verification === selectedStatus);
  //   }

  //   return filtered;
  // }, [searchTerm, selectedStatus, datas]);
  
  useEffect(() => {
    console.log("PRINCIPAL Text", principal?.toText());
  }, [principal]);

  if (loadingGetPatientList || patients === undefined) {
    return <UAMLoading />
  }

  return (
    <Flex w={'full'} flex={1}>
      <Flex flex={1} direction={'column'} p={10} gap={8}>
        <Text fontSize={'2xl'} fontWeight={'bold'}>
          {uamHeader.title}
        </Text>
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
              onClick={() => {}}
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
                  <UAMTable filter={filteredData} datas={patients} />
                  {/* <Pagination totalPages={datas.pagination.totalPage} /> */}
                </Flex>
              )
            }
          </Flex>
        </Flex>
      </Flex>

      {user && <UAMDetail getPatientList={getPatientList} />}
    </Flex>
  )
}

export default function UAMPageData({ user }: { user: string | null }) {
  return (
    <PatientActor canisterId={patientCanisterId}>
      <PatientList user={user} />
    </PatientActor>
  );
}