import { GetProviderBatchRequest, RegisternewProviderRequest } from "@/canister/declarations/provider_registry/provider_registry.did";
import { hospitalRegistrationForm } from "@/constants/contents/ham/form";
import { useProviderMethod, useProviderQuery } from "@/services/providers";
import { useProviderStore } from "@/store/providers.store";
import { useToast } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useSearchParams } from "next/navigation";
import { useState } from "react";

const useProvider = () => {
  const params = useSearchParams();
  const page = params.get('page') || "1";
  const limit = params.get('limit') || "10";
  
  const { success } = hospitalRegistrationForm;
  
  const toast = useToast();
  const { setProviders, setProvider } = useProviderStore()
  
  const [providerTotalPages, setProviderTotalPages] = useState(0)
  // const [providerTotalCount, setProviderTotalCount] = useState(0)
  
  const { call: getProviderList, loading: loadingProviderList } = useProviderQuery({
    functionName: "get_provider_list",
    refetchOnMount: true,
  });

  const handleGetProviderList = async () => {
    setProviders(undefined)

    await getProviderList([{ page : BigInt(Number(page) - 1), limit : BigInt(Number(limit)) }])
      .then((data) => {
        const result = data?.providers ?? [];
        setProviders(result);
        setProviderTotalPages(Number(data?.total_pages))
      })
      .catch((error) => {
        setProviders([])
        console.error(error);
      });
  };

  const { call: registerHospital, loading: registerHospitalLoading } = useProviderMethod({
    functionName: "register_new_provider",
    refetchOnMount: false,
    onSuccess() {
      toast({
        title: success.title,
        description: success.description,
        status: "success",
      });

      return;
    },
    onError(err) {
      throw err;
    },
  });

  const handleRegisterHospital = async (values: {
    name: string;
    address: string;
    principal: string;
  }, onClose: () => void) => {
    console.log(values)
    try {
      const principal = Principal.fromText(values.principal);

      const data: RegisternewProviderRequest = {
        address: values.address,
        display_name: values.name,
        provider_principal: principal,
      };

      await registerHospital([data]);
      await handleGetProviderList()

      onClose()
    } catch (error: unknown) {
      if (error instanceof Error) {
        toast({
          title: error.name,
          description: error.message,
          status: "error",
        });
      }

      console.error(error)
    }
  };

  const { call: getProviderInfo, loading: loadingProviderInfo } =
  useProviderQuery({
    functionName: "get_provider_batch",
    refetchOnMount: true,
  });

const handleProviderDetail = async (id: string) => {
  setProvider(undefined)

  const data: GetProviderBatchRequest = {
    ids: [id],
  };

  await getProviderInfo([data])
    .then((data) => {
      const result = data?.providers[0];
      if (result) setProvider(result);
    })
    .catch((error) => {
      setProvider(null)
      console.error(error);
    });
};
  
  return {
    loadingProviderList,
    registerHospitalLoading,
    loadingProviderInfo,
    providerTotalPages,
    handleGetProviderList,
    handleRegisterHospital,
    handleProviderDetail
  }
}

export default useProvider