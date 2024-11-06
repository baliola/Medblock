import { SuspendRequest } from "@/canister/declarations/provider_registry/provider_registry.did";
import { suspendHospitalText } from "@/constants/contents/ham/suspend";
import useProvider from "@/hooks/useProvider";
import { useProviderMethod } from "@/services/providers";
import { ISuspendUnsuspendButtonProps } from "@/utils/provider";
import { Button, useToast } from "@chakra-ui/react";
import { useUserPrincipal } from "@ic-reactor/react";
import { ReactElement } from "react";

const SuspendHospitalButton = ({ props }: { props: ISuspendUnsuspendButtonProps}): ReactElement => {
  const toast = useToast();
  const adminPrincipal = useUserPrincipal();
  const { handleGetProviderList, handleProviderDetail } = useProvider()

  const { call: suspendHospital, loading: suspendHospitalLoading } =
    useProviderMethod({
      functionName: "suspend_provider",
      refetchOnMount: false,

      onSuccess() {
        toast({
          title: suspendHospitalText.success.title,
          description: suspendHospitalText.success.description,
          status: "success",
        });

        return;
      },
      onError(err) {
        console.error(err);
        toast({
          title: suspendHospitalText.error.title,
          description: suspendHospitalText.error.description,
          status: "error",
        });

        return;
      },
    });

  const onSuspendHospital = async () => {
    try {
      if (adminPrincipal) {
        const data: SuspendRequest = {
          principal: props.principal,
        };

        await suspendHospital([data]);
        handleGetProviderList()
        handleProviderDetail(props.id)
      }
    } catch (error: unknown) {
      if (error instanceof Error) {
        toast({
          title: error.name,
          description: error.message,
          status: "error",
        });
      } else {
        toast({
          title: "Unknown error",
          description: "Something went wrong",
          status: "error",
        });
      }

      console.error(error);
    }
  };

  return (
    <Button
      w="full"
      rounded="xl"
      colorScheme="red"
      flex={1}
      alignItems={"center"}
      gap={3}
      py={3}
      color={"white"}
      onClick={onSuspendHospital}
      isLoading={suspendHospitalLoading}
    >
      Suspend
    </Button>
  );
};

export default SuspendHospitalButton;
