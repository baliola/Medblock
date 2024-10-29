import { Flex, Text } from "@chakra-ui/react";

import VitalSigns from "@/components/dashboard/emr/report/header/vital-sign";
import HospitalInfo from "@/components/dashboard/emr/report/header/hospital-info";
import VisitInfo from "@/components/dashboard/emr/report/header/visit-info";
import ActionButtons from "@/components/dashboard/emr/report/header/action-button";
import { useEMRDetail } from "@/store/patient-emr-detail";
import { ProviderActor } from "@/services/providers";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { emrDetailHeader } from "@/constants/contents/dashboard/emr/detail/header";

export default function HeaderEMRReport() {
  const emr = useEMRDetail(state => state.emr);
  if (!emr) return null;

  const emrData = emr.body.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);

  const vitalSigns = {
    blood_pressure: emrData["blood_pressure"] || "",
    heart_rate: emrData["heart_rate"] || "",
    respiration: emrData["respiration"] || "",
    temperature: emrData["temperature"] || "",
    oxygen_saturation: emrData["o2_saturation"] || "",
  };

  return (
    <Flex direction="column" w="full" gap={5}>
      <Text fontSize={{ base: 'md', lg: "lg" }} fontWeight={'bold'} color={'neutral.700'}>
        {emrDetailHeader.title}
      </Text>

      <Flex direction={'column'} gap={5} w={'full'} px={5}>
        <ProviderActor canisterId={providerCanisterId}>
          <HospitalInfo />
        </ProviderActor>

        <Flex align={'center'} color={"neutral.700"}>
          {emrDetailHeader.report.map((item, index) => (
            <VisitInfo key={index}
              title={item.title}
              value={emrData[item.key] || "-"}
            />
          ))}
        </Flex>

        <ActionButtons />
        <VitalSigns vitalSign={vitalSigns} />
      </Flex>
    </Flex>
  )
}
