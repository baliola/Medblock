"use client";

import { Flex, Text } from "@chakra-ui/react";
import { ProviderActor } from "@/services/providers";
import { useEffect } from "react";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { useAuthState, useUserPrincipal } from "@ic-reactor/react";
import { useProviderStore } from "@/store/providers.store";
import HAMTable from "./table";
import { hamHeader } from "@/constants/contents/ham/header";
import dynamic from "next/dynamic";
import HAMAddHospitalModal from "./add/hospital";
import { HAMLoading } from "./loading";
import useProvider from "@/hooks/useProvider";
import Pagination from "../pagination";
import { useSearchParams } from "next/navigation";

const HospitalList = () => {
  const params = useSearchParams();
  const page = params.get('page') || "1";
  const limit = params.get('limit') || "10";

  const principal = useUserPrincipal();
  const { authenticated } = useAuthState();
  const { providers } = useProviderStore();

  const { 
    loadingProviderList, 
    providerTotalPages,
    handleGetProviderList 
  } = useProvider()

  useEffect(() => {
    handleGetProviderList();
  }, []);

  useEffect(() => {
    console.log(providers)
  }, [providers])

  // useEffect(() => {
    console.log("PRINCIPAL Text", principal?.toText(), "authtenticated :", authenticated);
  // }, [principal]);

  if (loadingProviderList || providers === undefined) {
    return <HAMLoading />
  }

  return (
    <>
      {providers.length <= 0 ? (
        <Text fontSize={"md"} color={"neutral.700"} textAlign={"center"} py={6}>
          No data found.
        </Text>
      ) : (
        <Flex direction={"column"} gap={8} zIndex={0}>
          <HAMTable props={{ datas: providers, page: Number(page), limit: Number(limit)}} />
          <Pagination totalPages={providerTotalPages} />
        </Flex>
      )}
    </>
  );
};

function HAMData() {
  return (
    <ProviderActor canisterId={providerCanisterId}>
      <HospitalList />
    </ProviderActor>
  );
}

const NFIDButtonLogin = dynamic(() => import("@/components/auth/nfid/index"), {
  ssr: false,
});

const Header = () => (
  <Text fontSize={"2xl"} fontWeight={"bold"} w={"full"}>
    {hamHeader.title}
  </Text>
);

export default function HAMPageContent() {
  const { authenticated } = useAuthState();

  return (
    <Flex w={"full"} flex={1}>
      <Flex w={"full"} direction={"column"} p={10} gap={8}>
        <Flex justifyContent={"space-between"} alignItems={"center"} flexDirection={"column"} columnGap={8}>
          <Header />
          <Flex justifyContent={"end"} w={"full"}>
            <Flex w={"fit"}>
              {authenticated ? <HAMAddHospitalModal  /> : <NFIDButtonLogin />}
            </Flex>
          </Flex>
        </Flex>
        <HAMData />
      </Flex>
    </Flex>
  );
}
