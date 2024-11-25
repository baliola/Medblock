"use client";

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import { FaX } from "react-icons/fa6";
import { useProviderStore } from "@/store/providers.store";
import { ProviderActor } from "@/services/providers";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { ReactElement, useEffect } from "react";
import { getProviderStatusButton } from "@/utils/provider";
import { HAMDetailLoading } from "../loading";
import useProvider from "@/hooks/useProvider";
import { hamDetailProfile } from "@/constants/contents/ham/detail";

interface InfoRowProps {
  icon: React.ElementType;
  title: string;
  value?: string;
}

const InfoRow = ({ icon, title, value }: InfoRowProps) => {
  return (
    <Flex align={"center"} gap={5}>
      <Icon as={icon} boxSize={8} color={"neutral.700"} />
      <Flex direction={"column"} color={"neutral.700"}>
        <Text>{title}</Text>
        <Text fontSize={"lg"} fontWeight={"bold"} >
          {value}
        </Text>
      </Flex>
    </Flex>
  );
};

const DetailHospital = (): ReactElement => {
  const router = useRouter();
  const pathname = usePathname();
  const params = useSearchParams();
  const { provider, setProvider } = useProviderStore();
  const { loadingProviderInfo, handleProviderDetail } = useProvider()

  const param = new URLSearchParams(params);
  const id = param.get("hospital") as string;

  const onCloseDetail = () => {
    const param = new URLSearchParams(params);
    param.delete("hospital");

    const path = `${pathname}?${param.toString()}`;
    router.push(path);
  };

  useEffect(() => {
    handleProviderDetail(id);
  }, [id]);
  
  useEffect(() => {
    return () => {
      setProvider(undefined)
    }
  }, [])

  if (loadingProviderInfo || provider === undefined) return <HAMDetailLoading />;
  if (!provider) return <></>;

  return (
    <Flex
      w={"sm"}
      bg={"primary.100"}
      transition={"all 0.3s"}
      direction={"column"}
      p={7}
      gap={8}
      maxH={"100dvh"}
      overflowY={"auto"}
    >
      <Flex align={"center"} gap={5}>
        <Button
          aspectRatio={1}
          size={"xs"}
          rounded={"full"}
          colorScheme="red"
          p={0}
          display={"flex"}
          onClick={onCloseDetail}
        >
          <Icon
            as={FaX}
            width={"fit-content"}
            m={"auto"}
            display={"flex"}
            justifyContent={"center"}
            alignItems={"center"}
          />
        </Button>
        <Text fontSize={"2xl"} fontWeight={"bold"}>
          Hospital Details
        </Text>
      </Flex>

      <Flex direction={"column"} gap={8} flex={1}>
        <Flex direction={"column"} gap={5} ps={4}>
          <InfoRow
            icon={hamDetailProfile.hospital_name.icon}
            title={"Hospital Name"}
            value={provider.V1.display_name as string}
          />
          <InfoRow
            icon={hamDetailProfile.address.icon}
            title={"Hospital Address"}
            value={provider.V1.address as string}
          />
          <InfoRow
            icon={hamDetailProfile.registered_at.icon}
            title={"Registered At"}
            value={`${new Date (Number(provider.V1.registered_at) / 1e6).toDateString()}`}
          />
          <InfoRow
            icon={hamDetailProfile.principal.icon}
            title={"Principal"}
            value={provider.V1.provider_principal.toText()}
          />
        </Flex>
      </Flex>
      <Flex direction={"column"} gap={3}>
        {getProviderStatusButton(
          provider.V1.activation_status,
          provider.V1.provider_principal,
          id
        )}
      </Flex>
    </Flex>
  );
};

export default function HAMDetailHospital() {
  return (
    <ProviderActor canisterId={providerCanisterId}>
      <DetailHospital />
    </ProviderActor>
  );
}
