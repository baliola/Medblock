import { Flex, Grid, Text } from "@chakra-ui/react";
import EMRVitalSigns from "@/components/emr/vital-sign";
import { useEMRStore } from "@/store/emr-store";
import { emrDetailReports } from "@/constants/contents/emr/detail/reports";

export default function EMRReport() {
  const emr = useEMRStore((state) => state.emr);
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

  const SectionVisitSummary = ({ keys, title }: { keys: string; title: string }) => {
    const value = emrData[keys] || '';
    // if (!value) return null;
    return (
      <Flex direction="column" gap={2}>
        <Text color="neutral.500">{title}</Text>
        <Text color="neutral.700" whiteSpace={'pre-line'}>
          {value === '' || !value ? "-" : value}
        </Text>
      </Flex>
    );
  };

  return (
    <Flex direction="column" gap={5}>
      <EMRVitalSigns vitalSign={vitalSigns} />

      <Text fontSize="lg" fontWeight="bold" color="neutral.700">
        {emrDetailReports.header.report.title}
      </Text>

      <Grid templateColumns="repeat(2, 1fr)" gap={6}>
        {emrDetailReports.history.map((section, index) => (
          <SectionVisitSummary key={index}
            title={section.title}
            keys={section.key}
          />
        ))}
      </Grid>
      <Grid templateColumns="repeat(2, 1fr)" gap={6}>
        {emrDetailReports.allergies.map((section, index) => (
          <SectionVisitSummary key={index}
            title={section.title}
            keys={section.key}
          />
        ))}
      </Grid>
      {emrDetailReports.result.map((section, index) => (
        <SectionVisitSummary key={index}
          title={section.title}
          keys={section.key}
        />
      ))}
    </Flex>
  );
}
