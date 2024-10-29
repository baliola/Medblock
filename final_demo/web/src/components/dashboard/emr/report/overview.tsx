import { emrDetailReports } from "@/constants/contents/dashboard/emr/detail/reports";
import { useEMRDetail } from "@/store/patient-emr-detail";
import { Divider, Flex, Stack, Text } from "@chakra-ui/react";

export default function EMROverview() {
  const emr = useEMRDetail(state => state.emr);
  if (!emr) return null;

  const emrData = emr.body.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);

  const SectionVisit = ({ keys, title }: { keys: string; title: string }) => {
    const value = emrData[keys] || '';
    if (!value) return null;
    return (
      <Flex direction={"column"} gap={2} color={"neutral.500"}>
        <Text color={"neutral.500"} fontSize={'sm'}>
          {title}
        </Text>
        <Text
          fontSize={'md'}
          color={"neutral.700"}
          whiteSpace={'pre-line'}
        >
          {value ?? "-"}
        </Text>
      </Flex>
    )
  }

  return (
    <Stack divider={<Divider />} spacing={4}>
      {emrDetailReports.report.map((section, index) => (
        <SectionVisit key={index}
          title={section.title}
          keys={section.key}
        />
      ))}
    </Stack>
  )
}