#version 330 core
in vec3 fragNormals;
in vec3 fragPos;
in vec2 texCoords;

out vec4 FragColor;

struct Material {
  sampler2D diffuse;
  sampler2D specular;
  float shininess;
};

struct DirLight {
  vec3 direction;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct PointLight {
  vec3 position;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

struct SpotLight {
  vec3 position;
  vec3 direction;
  float cutOff;
  float outerCutOff;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

#define NR_POINT_LIGHTS 4

uniform vec3 viewPos;
uniform Material material;
uniform DirLight dirLight;
uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform SpotLight spotLight;

vec3 CalculateDirLight(DirLight light, vec3 normal, vec3 viewDir) {
  vec3 lightDir = normalize(-light.direction);

  float diff = max(dot(lightDir, normal), 0.0);

  vec3 reflectDir = reflect(-lightDir, normal);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

  vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));
  vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));

  return (ambient + diffuse + specular);
}

vec3 CalculatePointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
  vec3 lightDir = normalize(light.position - fragPos);

  float diff = max(dot(lightDir, normal), 0.0);

  vec3 reflectDir = reflect(-lightDir, normal);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

  // attenuation
  float distance = length(light.position - fragPos);
  float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance + distance));

  vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));
  vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}

vec3 CalculateSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
  vec3 lightDir = normalize(light.position - fragPos);

  float diff = max(dot(lightDir, normal), 0.0);

  vec3 reflectDir = reflect(-lightDir, normal);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

  // attenuation
  float distance = length(light.position - fragPos);
  float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance + distance));

  // spotlight
  float theta = dot(lightDir, normalize(-light.direction));
  float epsilon = light.cutOff - light.outerCutOff;
  float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

  vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));
  vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));


  diffuse *= intensity;
  specular *= intensity;

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}

void main()
{
  vec3 norm = normalize(fragNormals);
  vec3 viewDir = normalize(viewPos - fragPos);

  // direction light
  vec3 result = CalculateDirLight(dirLight, norm, viewDir);

  // point lights
  for (int i = 0; i < NR_POINT_LIGHTS; i++) {
    result += CalculatePointLight(pointLights[i], norm, fragPos, viewDir);
  }

  result += CalculateSpotLight(spotLight, norm, fragPos, viewDir);

  FragColor = vec4(result, 1.0);
}

