import { SuspendRequest } from "@/canister/declarations/provider_registry/provider_registry.did";
import { unsuspendHospitalText } from "@/constants/contents/ham/suspend";
import useProvider from "@/hooks/useProvider";
import { useProviderMethod } from "@/services/providers";
import { ISuspendUnsuspendButtonProps } from "@/utils/provider";
import { Button, useToast } from "@chakra-ui/react";
import { useUserPrincipal } from "@ic-reactor/react";
import { ReactElement } from "react";

const UnsuspendHospitalButton = ({ props }: { props: ISuspendUnsuspendButtonProps }): ReactElement => {
  const toast = useToast();
  const principal = useUserPrincipal();
  const { handleGetProviderList, handleProviderDetail } = useProvider()

  const { call: unsuspendHospital, loading: unsuspendHospitalLoading } =
    useProviderMethod({
      functionName: "unsuspend_provider",
      refetchOnMount: false,
      onSuccess() {
        toast({
          title: unsuspendHospitalText.success.title,
          description: unsuspendHospitalText.success.description,
          status: "success",
        });

        return;
      },
      onError(err) {
        console.error(err);
        toast({
          title: unsuspendHospitalText.error.title,
          description: unsuspendHospitalText.error.description,
          status: "error",
        });

        return;
      },
    });

  const onUnsuspendHospital = async () => {
    try {
      if (principal) {
        const data: SuspendRequest = {
          principal: props.principal,
        };

        await unsuspendHospital([data]);
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
      bg={"primary.700"}
      colorScheme="primary"
      flex={1}
      alignItems={"center"}
      gap={3}
      py={3}
      color={"white"}
      onClick={onUnsuspendHospital}
      isLoading={unsuspendHospitalLoading}
    >
      Unsuspend
    </Button>
  );
};

export default UnsuspendHospitalButton;
