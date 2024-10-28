"use client"

import { Fragment, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import { Flex, Icon, Skeleton } from "@chakra-ui/react";
import { FaRegShareFromSquare } from "react-icons/fa6";

import { useEMRStore } from "@/store/emr-store";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { EmrListPatientRequest } from "@/declarations/patient_registry/patient_registry.did";

import LoadingScreen from "@/layouts/loading";
import ShareConcentCode from "@/components/share-concent";
import EmptyHistoryEMR from "@/components/home/no-history";
import HomeHistoryVisits from "@/components/home/history-visits";
import HomePagination from "@/components/home/pagination";
import HomeHeader from "./header";
import { shareConcentButton } from "@/constants/contents/home/button";

const EMRHistory = () => {
  const setEMRS = useEMRStore(state => state.setEMRS);

  const searchParams = useSearchParams();
  const page = searchParams.get("page") || 0;
  const limit = searchParams.get("limit") || 10;

  const {
    call,
    loading,
    error: errorEmrList,
    data: userEMR
  } = usePatientQuery({
    functionName: "emr_list_patient",
    onSuccess(data) {
      console.log(data)
      // @ts-expect-error
      setEMRS(data?.emrs);
    },
    onError(error) {
      console.log(error)
    },
  });

  const onRefresh = () => {
    const request: EmrListPatientRequest = {
      page: Number(0),
      limit: Number(limit),
    };
    // @ts-expect-error
    call([request]);
  }

  useEffect(() => {
    const request: EmrListPatientRequest = {
      page: Number(0),
      limit: Number(limit),
    };
    // @ts-expect-error
    call([request]);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [page, limit]);

  // @ts-expect-error
  if (userEMR?.emrs.length === 0) {
    return <EmptyHistoryEMR refreshData={onRefresh} />
  }

  if (errorEmrList) {
    return <EmptyHistoryEMR refreshData={onRefresh} />
  }

  return (
    <Flex
      flex={1}
      direction={'column'}
      gap={5}
      justify={'space-between'}
    >
      <HomeHeader refreshData={onRefresh} />
      <Flex direction={'column'} gap={3} bg={'white'} flex={1}>
        {loading
          ? Array.from({ length: 3 }).map((_, index) => (
            <Skeleton key={index} w={'full'} h={24} rounded={"md"} />
          ))
          : (
            <Fragment>
              <HomeHistoryVisits />
              <HomePagination />
              <ShareConcentCode
                leftIcon={
                  <Icon as={FaRegShareFromSquare} boxSize={5} />
                }
              >
                {shareConcentButton.label}
              </ShareConcentCode>
            </Fragment>
          )
        }
      </Flex>
    </Flex>
  )
}

export default function HomeEMRHistory() {
  return (
    <PatientActor
      canisterId={patientCanisterId}
      loadingComponent={<LoadingScreen />}
    >
      <EMRHistory />
    </PatientActor>
  )
}